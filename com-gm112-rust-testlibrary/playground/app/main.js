import init, { add } from 'com-gm112-rust-testlibrary'

await init()
const result = add(2, 2)

console.log('Hello, World!')
console.log('2 + 2 =', result)

document.body.innerHTML = `
<h1>Hello, World!</h1>
<p>2 + 2 = ${result}</p>
`