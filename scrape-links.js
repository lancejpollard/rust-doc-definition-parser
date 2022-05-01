
let links = []
Array.prototype.slice.call(document.querySelectorAll('.structs a')).forEach(a => {
  links.push(a.href)
})
console.log(JSON.stringify(links, null, 2))

// https://microsoft.github.io/windows-docs-rs/doc/windows/all.html
