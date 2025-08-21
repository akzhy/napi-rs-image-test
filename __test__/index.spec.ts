import test from 'ava'
import path from 'path'

console.log(process.cwd())

test('sync function from native code', async (t) => {
  console.log(process.cwd())
  const { imageDimensions } = await import('../index.js')
  const image = path.join(process.cwd(), '__test__', 'image.avif')
  console.log(image)
  t.deepEqual(imageDimensions(image), [800, 800])
})
