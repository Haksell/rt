# rt

## todo

- [ ] read *The Ray Tracer Challenge*
- [ ] test speed of `f32` vs `f64`
- [ ] parallelize somehow (`#[repr(simd)]`, stdsimd, packed_simd, rayon?)
- [ ] read scene from JSON file
- [ ] reproduce 3 images from subject

## mandatory part

- [x] code in ~~C, C++ or~~ Rust
- [ ] implement the ray tracing method to create a computer generated image (important)
- [ ] geometric object: plane
- [ ] geometric object: sphere
- [ ] geometric object: cylinder
- [ ] geometric object: cone
- [ ] your program must be able to apply translation translation
- [ ] your program must be able to apply translation rotation
- [ ] position and direction of the camera can be changed easily
- [ ] manage to redraw the view or part of the view without recalculating the entire image (???)
- [ ] light management: different brightness
- [ ] light management: shadows
- [ ] light management: multi-spot
- [ ] light management: shine effect

## options

- [ ] limited objects: parallelograms, disks, half-spheres, tubes etc...
- [ ] composed elements: cubes, pyramids, tetrahedrons...
- [ ] more native elements: paraboloid, hyperboloid, tablecloth, toroid...
- [ ] ambiance light
- [ ] direct light
- [ ] parallel light
- [ ] bump mapping and color disruption
- [ ] external files for scene description
- [ ] reflection
- [ ] transparency
- [ ] shadow modification according to transparency of the elements
- [ ] textures
- [ ] negative elements
- [ ] limit disruption / transparency / reflection, depending on texture
