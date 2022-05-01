
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
  el = el.nextElementSibling
}

el = document.querySelector('#trait-implementations-list').firstElementChild
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

console.log(JSON.stringify({ title, sections }, null, 2))

function all(s, c) {
  return Array.prototype.slice.call((c || document).querySelectorAll(s))
}

function clean(x) {
  if (!x) return
  return x.replace(/[\s ]+/g, ' ').replace(/\s+Read more$/, '')
}
