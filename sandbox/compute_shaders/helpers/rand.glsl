// https://stackoverflow.com/questions/4200224/random-noise-functions-for-glsl
float PHI = 1.61803398874989484820459; // Golden ratio
float rand(in vec2 xy, in float seed) {
  vec2 pos = vec2(xy.x + 0.5, xy.y + 0.5);
  return fract(tan(distance(pos * PHI, pos) * seed) * pos.x);
}