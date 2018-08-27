/*cbuffer Constants {
    float4x4 Model;
    float4x4 View;
    float4x4 Projection;
}

struct VInput {
    float4 pos : POSITION;
    float2 uv : TEXCOORD;
};

struct VOutput {
    float4 pos : SV_Position;
    uint3 col : COLOR;
    float2 uv : TEXCOORD;
};

Texture2D<float4> Texture;
SamplerState Sampler;

VOutput vs(VInput input) {
    VOutput output;
    output.pos = mul(Projection, mul(View, mul(Model, float4(input.pos.xyz, 1))));
    output.uv = -abs(input.uv);
    output.col = saturate(Texture.SampleLevel(Sampler, float2(0,0), 0).rgb);
    return output;
}*/

float4 vs() : SV_Position {
    return 1.0.xxxx;
}
