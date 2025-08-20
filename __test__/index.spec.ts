import test from 'ava'
import path from 'path'

import { imageDimensions } from '../index'

test('sync function from native code', (t) => {
  const image = path.join(process.cwd(), '__test__', 'image.avif');
  console.log(image);
  t.deepEqual(imageDimensions(image), [800, 800])
})
