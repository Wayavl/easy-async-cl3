__kernel void fill_image(write_only image2d_t img_out) {
    int2 coord = (int2)(get_global_id(0), get_global_id(1));
    float4 red = (float4)(1.0f, 0.0f, 0.0f, 1.0f);
    write_imagef(img_out, coord, red);
}
