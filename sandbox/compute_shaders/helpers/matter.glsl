struct Matter
{
  uint color;
  uint state;
  uint matter;
  float weight;
  uint dispersion;
};

Matter new_matter(uint matter)
{
  Matter m;
  m.matter = (matter & uint(255));
  m.color = matter >> uint(8);
  m.state = matter_state[m.matter];
  m.weight = matter_weights[m.matter];
  m.dispersion = matter_dispersion[m.matter];
  return m;
}

uint matter_to_uint(Matter matter) { return ((matter.color << uint(8)) | matter.matter); }