import { ChessGame } from "chess_engine";
//import '@chrisoakman/chessboardjs';
//
var whiteSquareGrey = '#a9a9a9'
var blackSquareGrey = '#696969'

var game = ChessGame.new();

function removeGreySquares () {
  $('#myBoard .square-55d63').css('background', '')
}

function greySquare (square) {
  var $square = $('#myBoard .square-' + square)

  var background = whiteSquareGrey
  if ($square.hasClass('black-3c85d')) {
    background = blackSquareGrey
  }

  $square.css('background', background)
}

function onDrop (source, target, piece, newPos, oldPos, orientation) {
  removeGreySquares()

  console.log('Source: ' + source)
  console.log('Target: ' + target)
  console.log('Piece: ' + piece)
  console.log('New position: ' + Chessboard.objToFen(newPos))
  console.log('Old position: ' + Chessboard.objToFen(oldPos))
  console.log('Orientation: ' + orientation)
  console.log('~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~')
  var old_fen = Chessboard.objToFen(oldPos)
  if (game.check_if_legal(old_fen, source, target, piece)) {
     var new_fen = game.update(old_fen, source, target, piece)
     board.position(new_fen) 
     window.setTimeout(makeEngineMove, 1000)
  } else {
     return 'snapback'
  }
}

function makeEngineMove () {
  var fen = game.make_move(board.fen())//'r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R'
  board.position(fen)
}

var config = {
  draggable: true,
  dropOffBoard: 'snapback', // this is the default
  position: 'start',
  onDrop: onDrop,
  onMouseoutSquare: onMouseoutSquare,
  onMouseoverSquare: onMouseoverSquare
}
var board = Chessboard("myBoard", config)

function clickShowPositionBtn () {
  console.log('Current position as an Object:')
  console.log(board.position())

  console.log('Current position as a FEN string:')
  console.log(board.fen())
}

function onMouseoverSquare (square, piece) {
  // get list of possible moves for this square
  var piece_for_rust = "";
  if (piece != false) {
     piece_for_rust = piece;
  }
  var moves = game.get_legal_moves_for_highlighting(board.fen(), square, piece_for_rust)
  console.log(moves);

  // exit if there are no moves available for this square
  if (moves.length === 0) return

  // highlight the square they moused over
  greySquare(square)

  // highlight the possible squares for this piece
  for (var i = 0; i < moves.length; i++) {
    greySquare(moves[i])
  }
}

function onMouseoutSquare (square, piece) {
  removeGreySquares()
}

$('#showPositionBtn').on('click', clickShowPositionBtn)

/*const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);*/
