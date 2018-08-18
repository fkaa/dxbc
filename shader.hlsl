cbuffer Constants {
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
    float2 uv : TEXCOORD;
};

VOutput vs(VInput input) {
    VOutput output;

    output.pos = mul(Projection, mul(View, mul(Model, float4(input.pos.xyz, 1))));
    output.uv = abs(input.uv);

    return output;
}
