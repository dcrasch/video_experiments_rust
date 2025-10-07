// this variable will hold our shader object
let theShader;
let img;

function preload(){
  img = loadImage('test.png'); 

  // load the shader
  theShader = loadShader('assets/basic.vert', 'assets/basic.frag');
}

function setup() {
  // shaders require WEBGL mode to work
  createCanvas(710, 400, WEBGL);
  noStroke();
}

function draw() {
  // shader() sets the active shader with our shader

  let n = 6.0;
  theShader.setUniform('u_tex', img);
  theShader.setUniform('out_size',[2.0,2.0]);
  theShader.setUniform('in_size',[1,1] );
  theShader.setUniform('center_in',[0.5,0.5]);
  theShader.setUniform('center_out',[1.0,1.0]);
  theShader.setUniform('scale',2.5);
  theShader.setUniform('width',PI / n);
  theShader.setUniform('base_scale', 8.0);
  
  let t = millis() / 1000.0;
  theShader.setUniform('u_time',t);

  angle = (t / 15.0) * TWO_PI;
  // keep it in range [0, TWO_PI]
  angle = angle % TWO_PI;
  
  shift = sin(angle)*0.5
  theShader.setUniform('center_in',[0.5,1.0-shift]);
  theShader.setUniform('r_start',angle);
  theShader.setUniform('r_out',0);
  
    //let hue = (t * 60) % 360; // full cycle every 6 seconds
  let hue = ((sin(t) + 1.0) / 2.0) * 360.0; 
  let c = color(hue, 100, 100);
  let rgba_out = [red(c)/255.0, green(c)/255.0, blue(c)/255.0,1]; // normalized 0..1
  //let rgba_out=[0.0, 0.0, 1.0, 1.0];
theShader.setUniform("color_out",rgba_out);

  shader(theShader);

  // rect gives us some geometry on the screen
  rect(0,0,width, height);
}
