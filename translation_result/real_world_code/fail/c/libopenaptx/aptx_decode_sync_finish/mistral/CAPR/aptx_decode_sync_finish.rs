
struct AptxContext {
    decode_sync_buffer_len: usize,
    // other fields
}

fn aptx_decode_sync_finish(ctx: &mut AptxContext) -> usize {
    let dropped = ctx.decode_sync_buffer_len;
    aptx_reset(ctx);
    dropped as usize
}

fn aptx_reset(ctx: &mut AptxContext) {
    // implementation of aptx_reset
}
