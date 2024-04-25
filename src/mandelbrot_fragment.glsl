#version 300 es

precision highp float;
uniform vec2 canvasSize;
uniform vec4 viewportBounds;
uniform int MAX_ITERATIONS;
uniform float scaling_factor;
out vec4 outColor;
void main(){
    vec2 c=(gl_FragCoord.xy/canvasSize)*viewportBounds.yw+viewportBounds.xz;
    float x=0.;
    float y=0.;
    float tx=0.;
    int i=0;
    while((x*x)+(y*y)<=4.&&i<MAX_ITERATIONS){
        tx=(x*x)-(y*y)+c.x;
        y=2.*x*y+c.y;
        x=tx;
        i++;
    }
    outColor=vec4(float(i%255)/float(255),float(i*2%255)/float(255),float(i*3%255)/float(255),1.);
}