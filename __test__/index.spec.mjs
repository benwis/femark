import test from 'ava'

import { renderMarkdown } from '../index.js'

// Can it render a basic Hello World string?
// TODO: Add more tests
test('render Hello, World!', (t) => {
  t.is(renderMarkdown("# Hello, World!"), `<h1>Hello, World!</h1>\n`)
})
