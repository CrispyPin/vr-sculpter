extends Spatial

export var enable_highlight = false

onready var world = $"/root/Main/VoxelObject"
onready var player = $".."
onready var indicator = $"/root/Main/HighlightBox"
onready var chunkwire = $"/root/Main/ChunkHighlight"
var vtype := 255

var t_since_update := 0.0


func _process(delta):
	chunkwire.translation = (player.translation / 32).floor() * 32
	t_since_update += delta
	if t_since_update >= 0.3:
#		world.set_player_pos(player.translation)
		t_since_update = 0



	if Input.is_action_just_pressed("place"):
		world.brush_add(player.translation + forward() * 20, 15.0)

	if Input.is_action_just_pressed("break"):
		world.brush_add(player.translation + forward() * 20, 5.0)


func forward() -> Vector3:
	return (global_transform.origin - player.translation).normalized()
