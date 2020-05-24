module honey_comb(w, h, t, comb_d=5, comb_gap=0.5) {
  a = comb_d * cos(30);
  k = a + comb_gap;
  dx = k*cos(30);
  cols = floor(w / k);
  rows = floor(h / k);
  w2 = comb_d + (cols-1)*dx;
  h2 = floor(h*2/k)/2*k;
  translate([(w-w2)/2,(h-h2)/2,0]) intersection() {
    cube([w2, h2, t]);
    translate([comb_d/2, comb_d/2-(comb_d-k)/2, 0]) {
      for (x = [0: cols-1]) for (y = [0 : rows]) {
        px = x*dx;
        py = y*k - (x%2)/2*k;
        translate([px, py, 0]) cylinder(d=comb_d, $fn=6, h=t);
      }
    }
  }
}
