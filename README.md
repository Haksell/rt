# rt

## todo

- [ ] read *The Ray Tracer Challenge*
- [ ] implement operator overloading for `&Tuple` and `&Matrix` to avoid `.clone()` everywhere
- [ ] fix TODO
- [ ] clean code/architecture
- [ ] SIMD (`#[repr(simd)]`, `stdsimd`, `packed_simd`)
- [ ] multithreading (`std::thread`, `rayon`, `crossbeam`?)
- [ ] read scene from file (JSON?)
- [ ] reproduce 3 images from subject
- [ ] use `pub(crate)` where appropriate

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

- [ ] external files for scene description
- [ ] limited objects: parallelograms, disks, half-spheres, tubes etc...
- [ ] composed elements: cubes, pyramids, tetrahedrons...
- [ ] more native elements: paraboloid, hyperboloid, tablecloth, toroid...
- [ ] ambiance light
- [ ] direct light
- [ ] parallel light
- [ ] bump mapping and color disruption
- [ ] reflection
- [ ] transparency
- [ ] shadow modification according to transparency of the elements
- [ ] textures
- [ ] negative elements
- [ ] limit disruption / transparency / reflection, depending on texture

## bonus

- [ ] `.obj` files
- [ ] ellipsoids (scaled spheres)
