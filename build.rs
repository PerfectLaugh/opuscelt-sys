fn main() {
    let mut celt = cc::Build::new();

    celt.file("opus/celt/bands.c");
    celt.file("opus/celt/celt.c");
    celt.file("opus/celt/celt_encoder.c");
    celt.file("opus/celt/celt_decoder.c");
    celt.file("opus/celt/cwrs.c");
    celt.file("opus/celt/entcode.c");
    celt.file("opus/celt/entdec.c");
    celt.file("opus/celt/entenc.c");
    celt.file("opus/celt/kiss_fft.c");
    celt.file("opus/celt/laplace.c");
    celt.file("opus/celt/mathops.c");
    celt.file("opus/celt/mdct.c");
    celt.file("opus/celt/modes.c");
    celt.file("opus/celt/pitch.c");
    celt.file("opus/celt/celt_lpc.c");
    celt.file("opus/celt/quant_bands.c");
    celt.file("opus/celt/rate.c");
    celt.file("opus/celt/vq.c");

    celt.include("opus/include");
    celt.include("opus/celt");

    celt.define("USE_ALLOCA", None);
    celt.define("FIXED_POINT", None);
    celt.define("CUSTOM_MODES", None);

    celt.compile("celt");
}
