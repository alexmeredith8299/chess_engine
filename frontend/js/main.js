// NOTE: this example uses the chess.js library:
// https://github.com/jhlywa/chess.js
var board = null
var game = new Chess()

function testWasm(){  console.log('loading...');
  var testWasmVar = null;
  fetch('./../assets/engine.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {}))
  .then(wasmContainer => {
    const {add,subtract,multiply} = wasmContainer.instance.exports;
    console.log('4 + 2 = 6, which is the move we choose', add(4, 2));
//    console.log('4 - 2 = ', subtract(4, 2));
//    console.log('4 * 2 = ', multiply(4, 2));
    testWasmVar = add(4, 2);
    console.log(typeof testWasmVar);
  }).catch(err=>console.log(err));
  console.log(testWasmVar);
  return testWasmVar
}

function onDragStart (source, piece, position, orientation) {
  // do not pick up pieces if the game is over
  if (game.game_over()) return false

  // only pick up pieces for White
  if (piece.search(/^b/) !== -1) return false

  const test = testWasm();
  console.log(test);

}

function makeRandomMove () {
  var possibleMoves = game.moves()

  // game over
  if (possibleMoves.length === 0) return

  var randomIdx = Math.floor(Math.random() * possibleMoves.length)
  //var wasmIdx = testWasm()
  game.move(possibleMoves[randomIdx])
  board.position(game.fen())

}

function onDrop (source, target) {
  // see if the move is legal
  var move = game.move({
    from: source,
    to: target,
    promotion: 'q' // NOTE: always promote to a queen for example simplicity
  })

  // illegal move
  if (move === null) return 'snapback'

  // make random legal move for black
  window.setTimeout(makeRandomMove, 250)

}

// update the board position after the piece snap
// for castling, en passant, pawn promotion
function onSnapEnd () {
  board.position(game.fen())
}

var config = {
  draggable: true,
  position: 'start',
  onDragStart: onDragStart,
  onDrop: onDrop,
  onSnapEnd: onSnapEnd
}
board = Chessboard('myBoard', {
  draggable: true,
  position: 'start',
  onDragStart: onDragStart,
  onDrop: onDrop,
  onSnapEnd: onSnapEnd
})
