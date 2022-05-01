
const { paramCase } = require('change-case')
const mkdirp = require('mkdirp')
const fs = require('fs')
const puppeteer = require('puppeteer')
const urls = require('./rayon.links.json')
const NAME = `rayon`

mkdirp.sync(`docs/${NAME}`)

start()

async function start() {
  let b = await puppeteer.launch({ headless: true })
  let p = await b.newPage()
  await p.setUserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")

  const records = []

  while (urls.length) {
    await visit(urls.shift())
    const json = await p.evaluate(find)
    if (json) {
      const title = json.title.split(/\s+/).pop().replace(/:/g, '-')
      fs.writeFileSync(`docs/${NAME}/${title}.json`, JSON.stringify(json, null, 2))
    }
  }

  async function visit(u) {
    console.log(u)
    try {
      await p.goto(u)
    } catch (e) {
      console.log(e)
      try {
        await b.close()
      } catch (e) {
        b = await puppeteer.launch()
        p = await b.newPage()
      }
      await visit(u)
    }
  }

  await b.close()

  function find() {
    let title = document.querySelector('h1.fqn .in-band')?.textContent.trim()
    let el = document.querySelector('#implementations')?.nextElementSibling
    const methods = []
    while (el && el.tagName == 'DETAILS') {
      all('.rustdoc-toggle', el).forEach(x => {
        let code = clean(x.querySelector('.code-header')?.textContent.trim())
        if (!code) return
        let text = clean(x.querySelector('.docblock p')?.textContent.trim())
        let skip = !!x.querySelector('.portability, .deprecated')
        if (skip) return
        methods.push({
          code, text
        })
      })
      el = el?.nextElementSibling
    }

    el = document.querySelector('#trait-implementations-list')?.firstElementChild
    let traits = []
    while (el && el.tagName == 'DETAILS') {
      let title = clean(el.querySelector('h3.code-header')?.textContent.trim())
      let methods = []
      all('.rustdoc-toggle', el).forEach(x => {
        let code = clean(x.querySelector('.code-header')?.textContent.trim())
        if (!code) return
        let text = clean(x.querySelector('.docblock p')?.textContent.trim())
        let skip = !!x.querySelector('.portability, .deprecated')
        if (skip) return
        methods.push({
          code, text
        })
      })
      traits.push({
        title,
        methods
      })
      el = el.nextElementSibling
    }

    let sections = [
      {
        name: 'implementations',
        methods,
      },
      {
        name: 'derefMethods',
        methods: []
      },
      {
        name: 'traitImpl',
        traits
      }
    ]

    return { title, sections }

    function all(s, c) {
      return Array.prototype.slice.call((c || document).querySelectorAll(s))
    }

    function clean(x) {
      if (!x) return
      return x.replace(/[\s ]+/g, ' ').replace(/\s+Read more$/, '')
    }
  }
}
