__kernel void add(__global float* a, __global float* b) {
    int id = get_global_id(0);
    a[id] += b[id];
}
