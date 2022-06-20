import test from 'ava'

import { processMarkdownToHtml } from '../index.js'

test('hi', (t) => {
  t.is('hi', 'hi')
})
// Can it render a basic Hello World string?
// TODO: Add more tests
// test('render Hello, World!', (t) => {
//   t.is(processMarkdownToHtml("# Hello, World!").trim(),
//     `<h1><a id="hello-world" class="anchor" href="#hello-world">Hello, World!</a></h1>`)
// })
