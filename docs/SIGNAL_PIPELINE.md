# Signal Pipeline Boundary

Digital image processing is outside Team TMIG's substrate scope as a product feature, but it contributes a useful signal-processing grammar.

The pasted imaging pipeline collapses to five primitives:

1. Classification
2. Feature extraction
3. Multi-scale signal analysis
4. Pattern recognition
5. Projection

The important boundary is domain placement:

- physical/linear operations belong on the left side: linear RGB-like signals
- perceptual/encoded operations belong on the right side: gamma-corrected r'g'b, Y'CbCr, lossy coding

For TMIG this becomes a doctrine for artifact transforms: every transform must declare its domain before it is content-addressed.

Examples:

| Technique | Primitive sequence | Preferred domain |
|---|---|---|
| linear filtering / convolution | multi-scale analysis → feature extraction | linear |
| affine transform | projection | linear |
| morphology | feature extraction → pattern recognition | linear |
| wavelets | multi-scale analysis → feature extraction | linear |
| neural network | feature extraction → pattern recognition → classification | perceptual/encoded |
| pixelation / lossy coding | projection | perceptual/encoded |

Executable form: `src/signal_pipeline.rs`.
