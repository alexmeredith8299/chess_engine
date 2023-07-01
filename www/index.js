import { ChessGame } from "chess_engine";
//import '@chrisoakman/chessboardjs';
//
var whiteSquareGrey = '#a9a9a9'
var blackSquareGrey = '#696969'

var game = ChessGame.new();
var promote_to, promotion_dialog, move_cfg;

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
  var old_fen = Chessboard.objToFen(oldPos);
  if (game.check_if_legal(old_fen, source, target, piece)) {
     if (game.check_if_promotion(old_fen, source, target, piece)) {
	   move_cfg = {
		from: source,
		to: target,
		color: piece[0],
		promotion: 'q'
	    };
	    promotion_dialog = $('#promotion-dialog');
	    $('.promotion-piece-q').attr('src', getImgSrc('q', piece));
	    $('.promotion-piece-r').attr('src', getImgSrc('r', piece));
	    $('.promotion-piece-n').attr('src', getImgSrc('n', piece));
	    $('.promotion-piece-b').attr('src', getImgSrc('b', piece));

	    promotion_dialog.dialog({
	      modal: true,
	      height: 46,
	      width: 184,
	      resizable: true,
	      draggable: false,
	      close: onDialogClose,
	      closeOnEscape: false,
	      dialogClass: 'noTitleStuff'
	    }).dialog('widget').position({
	      of: $('#myBoard'),
	      my: 'middle middle',
	      at: 'middle middle',
	    });
	    return;
     } else {
	 var new_fen = game.update(old_fen, source, target, piece)
         board.position(new_fen, false) 
         //board.position(new_fen) 
         window.setTimeout(makeEngineMove, 1000)
     }
  } else {
     return 'snapback'
  }
}

function getImgSrc(piece, pawn) {
   return 'img/chesspieces/wikipedia/' + pawn[0] + piece.toLocaleUpperCase() + '.png'
}

function makeEngineMove () {
  var fen = game.make_move(board.fen())//'r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R'
  board.position(fen)
}

function pieceTheme (piece) {
  // wikipedia theme for white pieces
  return 'img/chesspieces/wikipedia/' + piece + '.png'
}

var config = {
  draggable: true,
  dropOffBoard: 'snapback', // this is the default
  position: 'start',
  onDrop: onDrop,
  onMouseoutSquare: onMouseoutSquare,
  onMouseoverSquare: onMouseoverSquare,
  pieceTheme: pieceTheme 
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
var onDialogClose = function() {
   console.log(promote_to);
   var piece = move_cfg.color + promote_to.toLocaleUpperCase();
   var new_fen = game.update(board.fen(), move_cfg.from, move_cfg.to, piece)
   board.position(new_fen, false) 

}
$('#showPositionBtn').on('click', clickShowPositionBtn)
// init promotion piece dialog
//
$("#promote-to").selectable({
  stop: function() {
    $( ".ui-selected", this ).each(function() {
      var selectable = $('#promote-to li');
      var index = selectable.index(this);
      if (index > -1) {
	var promote_to_html = selectable[index].innerHTML;
	var span = $('<div>' + promote_to_html + '</div>').find('span');
	promote_to = span[0].innerHTML;
      }
      promotion_dialog.dialog('close');
      $('.ui-selectee').removeClass('ui-selected');
      //updateBoard(board);
    });
  }
});

/*const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);*/
