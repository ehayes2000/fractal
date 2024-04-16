#version 300 es

precision mediump float;
uniform vec2 canvasSize;
uniform vec4 viewportBounds;
uniform int MAX_ITERATIONS;
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
    float ipercent=float(i)/float(MAX_ITERATIONS);
    outColor=vec4(ipercent/4.,ipercent/2.,ipercent,1.);
}