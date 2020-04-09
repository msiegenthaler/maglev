// Duplo-compatible block library (C) by Joachim Eibl 2013
// Licence: CC BY-NC-SA 3.0 
// Creative Commons: attribution, non commercial, share alike
// See: http://creativecommons.org/licenses/by-nc-sa/3.0/
// This lib is derived from http://www.thingiverse.com/thing:1778
// LEGO and DUPLO are trademarks of the Lego group.


//the duplo itself
// parameters are: 
// width: 1 =standard 4x4 duplo width.
// length: 1= standard 4x4 duplo length
// height: 1= minimal duplo height

duploRaster = 16;
dr = duploRaster;
duploHeight=duploRaster * 0.6;



// nibble radius: a square brick with size (dr x dr) turned by 45° on a neighbor nibble fits exactly:
duploNibbleRadius = dr * (1-1/1.41421) +0.1; // = 4.686+0.1,  dr*(1-1/sqrt(2))
duploBottomNibbleRadius = dr*(1.41421-1); //= dr * 1.41421 / 2  - duploNibbleRadius = 6.6277
duploGapBottom = -0.05; // recommended range from -0.1 to 0.1 with lower values for tighter fit.
gapBetweenBricks = 0.3; // real duplo probably has 0.4
duploWall = 1.55;// For duplo compatibility this is not so important, only if smaller lego should fit.
                   // (dr/2 - duploNibbleRadius - gapBetweenBricks)/2 = 1.507
firstLayerGap = 0.2; // for easier fit and to compensate for printers that print a thicker bottom
firstLayerGapHeight = 0.3;
bottomGapHeight = 0;

quality = 60; // quality: low/fast (e.g. 10) for design, high/slow (e.g. 50) for final rendering 


// if some pieces are missing: "Edit"->"Preferences"->"Advanced"->"Turn off rendering at: " [1000000] "elements"


//duplo(2,2,1,true,false);



module duplo(width,length,height,topNibbles,bottomHoles) 
{
   //size definitions
   
   ns = duploRaster / 2;  //nibble start offset
   nbo = duploRaster;  // nibble bottom offset
   effWidth = width * duploRaster-gapBetweenBricks;
   effLength = length*duploRaster-gapBetweenBricks;
   littleWallThickness = 2.35; // 1.35 is standard but bigger is better for printing

   //the cube
   difference() {
      cube([effWidth,effLength,height*duploHeight],true);
      translate([0,0,-duploWall])    
         cube([width*duploRaster - 2*duploWall,length*duploRaster-2*duploWall,height*duploHeight],true);
   }

   if(topNibbles)
   {
      //nibbles on top
      for(j=[1:length])
      {
         for (i = [1:width])
         {
            // disabled
            translate([ns-(i-width*0.5)*dr,ns-(j-length*0.5)*dr,6.9+(height-1)*duploHeight/2]) duplonibble();
         }
      }
   }

   if(bottomHoles)
   {
      difference() {
         cube([effWidth,effLength,height*duploHeight],true);
      
         //nibbles on bottom
         for(j=[1:length])
         {
            for (i = [1:width])
            {
               // disabled
               translate([ns-(i-width*0.5)*dr,ns-(j-length*0.5)*dr,-0.1-height*duploHeight/2])
               {
                  cylinder(r=duploNibbleRadius+0.2,h=6,center=false,$fn = quality);
                  cylinder(r=duploNibbleRadius+0.4,h=0.5,center=false,$fn = quality);
               }
            }
         }
      }
   }
   else
   {
   //nibble bottom
   if ( length > 1 && width > 1 )
   {
      for(j=[1:length-1])
      {
         for (i = [1:width-1])
         {
            translate([(i-width*0.5)*dr,(j-length*0.5)*dr,0]) duplobottomnibble(height*duploHeight);
         }
      }
   }
   //little walls inside
   difference() 
   {
      union()
       {
         for(j=[1:length])
         {   
            translate([0,ns-(j-length/2)*dr,0 ]) cube([effWidth,littleWallThickness,height*duploHeight],true);
         }
         for (i = [1:width])
         {
            translate([ns-(i-width/2)*dr,0,0 ]) cube([littleWallThickness,effLength,height*duploHeight],true);
         }
         for(j=[1:length-1])
         {   
            if (width==1)
               translate([0,ns-(j-length/2+0.5)*dr,0 ]) cube([effWidth,littleWallThickness,height*duploHeight],true);
         }
      }
      if ( width > 1 )
      {
         cube([(width-1)*dr + duploNibbleRadius*2+duploGapBottom,(length-1)*dr+duploNibbleRadius*2+duploGapBottom,height*duploHeight+2],true);
         translate([0,0,-height*duploHeight/2+firstLayerGapHeight/2]) 
         cube([(width-1)*dr + duploNibbleRadius*2+duploGapBottom+firstLayerGap,(length-1)*dr+duploNibbleRadius*2+duploGapBottom+0.2,firstLayerGapHeight+0.01],true);
      }
      else
         for(j=[1:length])
         {   
            translate([0,(+j-length/2 - 0.5)*dr,0 ])
               cube([ duploNibbleRadius*2+duploGapBottom,duploNibbleRadius*2+duploGapBottom,height*duploHeight+2],true);
         }
   }
   }
}


module duplonibble()
{
   difference() {
      union() {
         translate([0,0,-0.5/2]) cylinder(r=duploNibbleRadius,h=4.5-1,center=true,$fn = quality);
         translate([0,0,4.5/2-1]) cylinder(r1=duploNibbleRadius,r2=duploNibbleRadius-0.2,h=1,$fn = quality);
      }
      cylinder(r=duploNibbleRadius-1.3,h=5.5,center=true,$fn = quality);
   }
}

module duplobottomnibble(height)
{
   difference() {
      union(){
         cylinder(r=duploBottomNibbleRadius-firstLayerGap,h=height,center=true,$fn = quality);
         translate([0,0,bottomGapHeight/2]) cylinder(r=duploBottomNibbleRadius,h=height-firstLayerGapHeight,center=true,$fn = quality);
      }
      cylinder(r=duploBottomNibbleRadius-1.3,h=height+1,center=true,$fn = quality);
   }

}
