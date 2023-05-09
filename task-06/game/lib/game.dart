import 'dart:ui';
import 'package:flame/game.dart';
import 'player.dart';
import 'world.dart';
import 'directions.dart';

class BunnyGame extends FlameGame {
  final Player _player = Player();
  final World _world = World();

  onArrowKeyChanged(Direction direction) {
    _player.direction = direction;
  }

  @override
  Future<void> onLoad() async {
    super.onLoad();
    await add(_world);
    await add(_player);
    _player.position = Vector2(0, 895);
    camera.followComponent(_player,
        worldBounds: Rect.fromLTRB(0, 0, _world.size.x, _world.size.y));
  }
}
