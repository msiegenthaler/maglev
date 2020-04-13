include <lib/duplo-block-lib.scad>

$fs = 0.1;

delta = 0.005; //for better preview rendering. set to 0 for 'perfect' 
magnet_w = 4;
magnet_h = 4;
magnet_l = 15;



// duplo_bottom(4, 3);
// duplo_top_straight(4, 3);
// duplo_top_cross(2, 4);
// duplo_top_cross(4, 8);
// duplo_vert_vert(2, 4);
// duplo_railholder_a(4, 4, 2);
// translate([0,0,30])
  // duplo_bottom(4, 3);

rotate([0,90,0]) {
  soleniod(3, 1.5, left=true);
  translate([0,25,0]) soleniod(3, 1.5, left=false);
}

//magnet();
//test_stripe();

// Duplo stick of size lenx1 that contains count magnets and attaches to the bottom
module duplo_bottom(len=4, count=3) {
  mag_y = (duploRaster*len-gapBetweenBricks)/count;
  th = magnet_h+0.7+0.6;
  difference() {
    union() {
      translate([0,0,-6])
        duplo(1,len,1,0);
      translate([0,0,-th/2])
        cube([duploRaster-gapBetweenBricks,duploRaster*len-gapBetweenBricks,th], center=true);
    }
    for (i=[0:count-1]) {
      translate([0, +mag_y*((count-1)/2-i), delta]) magnet_pit();
    }
  }
}

// Duplo stick of size lenx1 that contains count magnets and attaches to the top
module duplo_top_straight(len=4, count=3) {
  mag_y = (duploRaster*len-gapBetweenBricks)/count;
  th = magnet_h+0.7+0.6;
  difference() {
    union() {
      translate([0,0,1]) rotate([180,0,0]) duplo(1, len, 0, 1);
      translate([0,0,th/2])
      cube([duploRaster-gapBetweenBricks,duploRaster*len-gapBetweenBricks,th], center=true);
    }
    for (i=[0:count-1]) {
      translate([0, +mag_y*((count-1)/2-i), th+delta]) magnet_pit();
    }
  }
}

// Duplo stick of size lenx2 that contains count magnets and attaches to the top
module duplo_top_cross(len=4, count=8) {
  mag_y = (duploRaster*len-gapBetweenBricks)/count;
  th = magnet_h+0.7+0.6;
  difference() {
    union() {
      translate([0,0,1]) rotate([180,0,0]) duplo(2, len, 0, 1);
      translate([0,0,th/2])
        cube([duploRaster*2-gapBetweenBricks,duploRaster*len-gapBetweenBricks,th], center=true);
    }
    translate([-duploRaster/2+0.6,0,th+delta]) {
      for (i=[0:count-1]) {
        translate([0, +mag_y*((count-1)/2-i), 0]) rotate([0,0,90]) magnet_pit();
      }
    }
  }
}

// Duplo stick of size lenx1x2 that contains count vertical magnets and attaches to the bottom
module duplo_vert_vert(len=2, count=4) {
  mag_y = (duploRaster*len-gapBetweenBricks)/count;
  th = duploHeight*2;
  dz = 0;
  *translate([0,0,-duploRaster/2+dz+3.7])
    cube([duploRaster-gapBetweenBricks, duploRaster*2-gapBetweenBricks, 2*duploHeight-gapBetweenBricks], center=true);
  difference() {
    union() {
      translate([0,0,-dz-duploHeight/2])
        duplo(1,len,3,1);
      translate([duploRaster/4,0,-th/2+duploHeight])
        cube([duploRaster/2-gapBetweenBricks,duploRaster*len-gapBetweenBricks,th], center=true);
    }
    for (i=[0:count-1]) {
      translate([duploRaster/2-gapBetweenBricks/2+delta, +mag_y*((count-1)/2-i), 0]) rotate([90,0,90]) magnet_pit();
    }
  }
}

// railholder with arms that are l+r on top of the rail
module duplo_railholder_a(len, magnet_count, arm_count) {
  arm_w = 10;
  module arm(alpha, k) {
    block_h = magnet_h+1.3;
    inner_h = 14.5;       inner_w=inner_h/tan(alpha);
    mag_hold_depth = 5.1; mag_holder_x=mag_hold_depth*sin(alpha); mag_holder_y=mag_hold_depth*cos(alpha);
    translate([(duploRaster-gapBetweenBricks)/2,0,0]) {
      difference() {
        rotate([90,0,0]) linear_extrude(arm_w, center=true) {
          polygon([[0,0], [0,block_h], [inner_w,inner_h+block_h],
                  [inner_w+mag_holder_x, inner_h+block_h-mag_holder_y],
                  [mag_hold_depth*cos(90-alpha), 0],
                  [0,0]]);
        }
        rotate([0, -alpha, 0]) translate([13,0,2.7+k]) rotate([0,0,90]) magnet_pit();
      }
    }
  }

  y_total = (duploRaster*len-gapBetweenBricks)-arm_w;
  union() {
    duplo_top_straight(len, magnet_count);
    if (arm_count == 1) {
      arm(55, 0.4);
      mirror([1,0,0]) arm(55, 0.4);
    } else {
      for (i=[1:arm_count]) {
        translate([0,y_total/2-y_total/(arm_count-1)*(i-1),0]) {
          arm(55, 0.4);
          mirror([1,0,0]) arm(55, 0.4);
        }
      }
    }
  }
}

module soleniod(size=1, len=1, left=false) {
  inside_d = 6.2+1*(size-1); outside_d = 12+4*(size-1);
  wall = 1.2; side_wall=0.8; gap = 0.08;
  w = duploRaster*len-2*0.6;
  holder_w = 2;
  rotate([0,90,0]) difference() {
    union() {
      cylinder(w/2, d=inside_d, center=true);
      translate([0,0,w/4-side_wall/2]) cylinder(side_wall, d=outside_d, center=true);
      if (!left)
        translate([0,0,-w/4-holder_w/2+gap]) cylinder(holder_w-gap*2, d=inside_d-wall, center=true);
    }
    if (left) {
      translate([0,0,-w/4+holder_w/2-delta]) cylinder(holder_w+2*delta, d=inside_d-wall-gap, center=true);
    }
    translate([0,0,delta-holder_w/2]) cylinder(w/2+delta*4+holder_w, d=inside_d-wall*2, center=true);
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