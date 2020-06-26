#version 330

out vec4 outputColor;

uniform float fragLoopDuration;
uniform float time;

const vec4 firstColor = vec4(0.0f, 1.0f, 1.0f, 1.0f);
const vec4 secondColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);

void main()
{
    float currTime = mod(time, fragLoopDuration);
    float timeScale = 3.14159f * 2.0f / fragLoopDuration;
    float currLerp = (sin(currTime * timeScale) + 1) / 2;

    outputColor = mix(firstColor, secondColor, currLerp);
}