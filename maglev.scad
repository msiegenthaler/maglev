include <lib/duplo-block-lib.scad>

$fs = 0.1;

delta = 0.005; //for better preview rendering. set to 0 for 'perfect' 
magnet_w = 4;
magnet_h = 4;
magnet_l = 15;



//magnet();
// duplo_bottom_4_high();
duplo_bottom_4_low();

module duplo_bottom_4_high() {
  len = 4;
  mag_y = (duploRaster*len-gapBetweenBricks)/4;
  th = magnet_h;
  difference() {
    union() {
      translate([0,0,-8.7])
        duplo(1,len,1,0);
      translate([0,0,-th/2])
        color("red") cube([duploRaster-gapBetweenBricks,duploRaster*len-gapBetweenBricks,th], center=true);
    }
    translate([0,mag_y/2,delta]) {
      translate([0, +mag_y*1, 0]) magnet_pit();
      translate([0, -mag_y*1, 0]) magnet_pit()
      translate([0, -mag_y*2, 0]) magnet_pit();
    }
  }
}
module duplo_bottom_4_low() {
  len = 4;
  mag_y = (duploRaster*len-gapBetweenBricks)/3;
  th = magnet_h+0.7+0.6;
  difference() {
    union() {
      translate([0,0,-6])
        duplo(1,len,1,0);
      translate([0,0,-th/2])
        cube([duploRaster-gapBetweenBricks,duploRaster*len-gapBetweenBricks,th], center=true);
    }
    translate([0,0,delta]) {
      translate([0, +mag_y*1, 0]) magnet_pit();
      magnet_pit();
      translate([0, -mag_y*1, 0]) magnet_pit();
    }
  }
}


module test_stripe() {
  wall = 1;
  count = 3;
  difference() {
    translate([-magnet_w/2-wall,-wall,-6-delta])
      cube([magnet_w+2*wall,(magnet_l+2*wall)*count,6]);
    translate([0,magnet_l/2,0]) {
      magnet_pit(hole_l=5);
      translate([0,magnet_l+2*wall,0])
        magnet_pit(hole_l=5);
      translate([0,2*(magnet_l+2*wall),0])
        magnet_pit(hole_l=5);
    }
  }
}

module magnet_pit(hole_l=0) {
  gap_w=0.03; gap_l=0.15; gap_h=0.2;
  w = magnet_w+2*gap_w; l = magnet_l+2*gap_l*2; h = magnet_h+2*gap_h;
  teeth_inset = 0.5; teeth_width = 3; teeth_thickness = 0.3;
  tf_w = 1; tf_l = 0.0;
  union() {
    translate([0,0,-h/2-teeth_thickness+delta])
      cube([w, l, h], center=true);
    translate([0,0,-teeth_thickness/2]) {
      difference() {
        cube([w,l,teeth_thickness], center=true);
        translate([0,0,0.01]) {
          translate([-(w-teeth_inset)/2-delta,-l/4,0]) cube([teeth_inset,tf_w,teeth_thickness], center=true);
          translate([-(w-teeth_inset)/2-delta,0,0]) cube([teeth_inset,tf_w,teeth_thickness], center=true);
          translate([-(w-teeth_inset)/2-delta,l/4,0]) cube([teeth_inset,tf_w,teeth_thickness], center=true);
          translate([(w-teeth_inset)/2-delta,-l/4,0]) cube([teeth_inset,tf_w,teeth_thickness], center=true);
          translate([(w-teeth_inset)/2-delta,0,0]) cube([teeth_inset,tf_w,teeth_thickness], center=true);
          translate([(w-teeth_inset)/2-delta,l/4,0]) cube([teeth_inset,tf_w,teeth_thickness], center=true);
          translate([0,-(l-teeth_inset)/2-delta,0]) cube([tf_w,teeth_inset,teeth_thickness], center=true);
          translate([0,(l-teeth_inset)/2+delta,0]) cube([tf_w,teeth_inset,teeth_thickness], center=true);
        }
      }
    }
    if (hole_l > 0) {
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