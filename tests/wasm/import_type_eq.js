class G {}
class H extends G {}

exports.G = G;
exports.H = H;
exports.sameH = h => h;
exports.newG = () => new G();
