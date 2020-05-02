plug_h = 9;
plug_w = 18;
plug_d = 9;
plug_ratch_z = 2.7;
plug_ratch_h = 3.3;
plug_ratch_d = 6.7;
plug_wall = 1.3;
plug_holder_wall = 0.5;
plug_holder_gap = 0.11;

delta = $preview ? 0.005 : 0; //for better preview rendering

plug_holder_y_offset = plug_h/2 + plug_holder_wall;
plug_holder_y_size = plug_h + plug_holder_wall;
plug_holder_x_size = plug_w + 2*plug_holder_wall + 2*plug_holder_gap;
plug_holder_z_size = plug_d + 2*plug_holder_wall;

module plug() {
  difference() {
    cube([plug_w, plug_h, plug_d], center=true);
    translate([0,0,plug_ratch_z/2+delta])
      cube([plug_w-2*plug_wall, plug_h-2*plug_wall, plug_d-plug_ratch_z], center=true);
    
    translate([plug_w/2-plug_wall/2,0,plug_ratch_d/2-plug_d/2-delta])
      cube([plug_wall+2*delta, plug_ratch_h, plug_ratch_d], center=true);
    translate([-plug_w/2+plug_wall/2,0,plug_ratch_d/2-plug_d/2-delta])
      cube([plug_wall+2*delta, plug_ratch_h, plug_ratch_d], center=true);
  }
}

module latch(height, width, depth) {
  translate([width/2,0,-height]) rotate([0,-90,0]) linear_extrude(width) 
    polygon(points=[[0,0], [height,0], [height,depth], [0,0]]);
}

module plug_holder() {
  wall = plug_holder_wall;
  gap = plug_holder_gap;
  stud_x = plug_wall-0.2;

  w = plug_holder_x_size;
  h = plug_h + 2*wall + 2*gap;
  b_w = 2;
  union() {
    difference() {
      translate([0,0,-wall/2])
        cube([w, h, plug_d+wall], center=true);
      cube([plug_w+2*gap, plug_h+2*gap, plug_d+2*delta], center=true);
      cube([plug_w-2*b_w, h-2*wall, plug_d*2], center=true);
    }
    translate([stud_x/2-w/2+wall,0,+plug_ratch_d/2-plug_d/2])
      cube([stud_x, plug_ratch_h-gap*2, plug_ratch_d], center=true);
    translate([-plug_w/2-gap+stud_x,0,-plug_d/2+plug_ratch_z]) rotate([0,180,-90])
      latch(0.5, 1, 0.3);
    translate([-stud_x/2+w/2-wall,0,+plug_ratch_d/2-plug_d/2])
      cube([stud_x, plug_ratch_h-gap*2, plug_ratch_d], center=true);
    translate([plug_w/2+gap-stud_x,0,-plug_d/2+plug_ratch_z]) rotate([0,180,90])
      latch(0.5, 1, 0.3);
  }

  // %plug();
}

module plug_holder_box() {
  w = plug_holder_x_size;
  h = plug_holder_y_size+plug_holder_wall;
  d = plug_d+plug_holder_wall;
  translate([0,0,-plug_holder_wall/2]) cube([w,h,d], center=true);
}
