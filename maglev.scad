$fs = 0.1;

delta = 0.005; //for better preview rendering. set to 0 for 'perfect' 
magnet_w = 4;
magnet_h = 4;
magnet_l = 15;



//magnet();

wall = 1;
count = 3;
difference() {
  translate([-magnet_w/2-wall,-wall,-6-delta])
    cube([magnet_w+2*wall,(magnet_l+2*wall)*count,6]);
  translate([0,magnet_l/2,0]) {
    magnet_pit(0.3, hole=true);
    translate([0,magnet_l+2*wall,0])
      magnet_pit(0.5, hole=true);
    translate([0,2*(magnet_l+2*wall),0])
      magnet_pit(0.7, hole=true);
  }
}




module magnet_pit(teeth_inset, hole=false) {
  gap_w=0.03; gap_l=0.15; gap_h=0.2;
  w = magnet_w+2*gap_w; l = magnet_l+2*gap_l*2; h = magnet_h+2*gap_h;
  // teeth_inset = 0.2;
  teeth_width = 3; teeth_thickness = 0.3;
  tf_w = 0.5; tf_l = 0.8;
  union() {
    translate([0,0,-h/2-teeth_thickness+delta])
      cube([w, l, h], center=true);
    translate([0,0,-teeth_thickness/2]) {
      difference() {
        cube([w,l,teeth_thickness], center=true);
        translate([0,0,0.01]) {
          translate([-(w-teeth_inset)/2-delta,0,0]) cube([teeth_inset,l*tf_l,teeth_thickness], center=true);
          translate([(w-teeth_inset)/2+delta,0,0]) cube([teeth_inset,l*tf_l,teeth_thickness], center=true);
          translate([0,-(l-teeth_inset)/2-delta,0]) cube([w*tf_w,teeth_inset,teeth_thickness], center=true);
          translate([0,(l-teeth_inset)/2+delta,0]) cube([w*tf_w,teeth_inset,teeth_thickness], center=true);
        }
      }
    }
    if (hole) {
      hole_l = 5;
      translate([0,0,-h-hole_l/2-teeth_thickness+delta*2])
        cylinder(hole_l, d=2, center=true);
    }
  }
}

module magnet(top=-1) {
  translate([-magnet_w/2,-magnet_l/2,-magnet_h/2+magnet_h/2*top]) {
    color("red") cube([magnet_w, magnet_l, magnet_h/2]);
    color("green") translate([0,0, magnet_h/2]) cube([magnet_w, magnet_l, magnet_h/2]);
  }
}