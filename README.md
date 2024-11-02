# rt

## todo

- [ ] read *The Ray Tracer Challenge*
- [ ] implement operator overloading for `&Tuple` and `&Matrix` to avoid `.clone()` everywhere
- [ ] fix TODO
- [ ] clean code/architecture
- [ ] multithreading (`std::thread`)
- [ ] SIMD (`#[repr(simd)]`, `stdsimd`, `packed_simd`)
- [ ] GPU (`wgpu`?)
- [ ] read scene from file (JSON?)
- [ ] reproduce 3 images from subject

## objects

- [x] geometric object: plane
- [x] geometric object: sphere
- [ ] geometric object: cylinder
- [ ] geometric object: cone
- [ ] limited object: parallelogram
- [ ] limited object: disk
- [ ] limited object: half-sphere
- [ ] limited object: tube
- [ ] composed element: cube
- [ ] composed element: pyramid
- [ ] composed element: tetrahedron
- [ ] native element: ellipsoid (just scaled spheres?)
- [ ] native element: paraboloid
- [ ] native element: hyperboloid
- [ ] native element: tablecloth
- [ ] native element: toroid

## mandatory part

- [x] code in ~~C, C++ or~~ Rust
- [ ] implement the ray tracing method to create a computer generated image
- [x] your program must be able to apply translation
- [x] your program must be able to apply rotation
- [ ] position and direction of the camera can be changed easily
- [ ] manage to redraw the view or part of the view without recalculating the entire image (???)
- [ ] light management: different brightness
- [ ] light management: shadows
- [ ] light management: multi-spot
- [ ] light management: shine effect

## options

- [ ] external files for scene description
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

- [x] pattern: stripe
- [ ] pattern: checkerboard
- [ ] pattern: gradient
- [ ] `.obj` files