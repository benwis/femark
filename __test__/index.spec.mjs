import test from 'ava'

import { renderMarkdown } from '../index.js'

test('render Hello World', (t) => {
  t.is(renderMarkdown("# Hello, world"), `<h1>Hello, world</h1>\n`)
})
