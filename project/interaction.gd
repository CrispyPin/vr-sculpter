extends Spatial

export var enable_highlight = false

onready var world = $"/root/Main/VoxelObject"
onready var player = $".."
onready var indicator = $"/root/Main/HighlightBox"
onready var chunkwire = $"/root/Main/ChunkHighlight"
var vtype := 255

var t_since_update := 0.0

func _ready():
	world.set_sphere(Vector3(0,0,0), 32.0, 255)
	world.smooth_sphere(Vector3(0,0,0), 34.0)


func _process(delta):
	chunkwire.translation = (player.translation / 32).floor() * 32
	t_since_update += delta
	if t_since_update >= 0.3:
#		world.set_player_pos(player.translation)
		t_since_update = 0



	if Input.is_action_pressed("place"):
		world.set_sphere(player.translation + forward() * 20, 5.0, 255)
		world.smooth_sphere(player.translation + forward() * 20, 7.0)

	if Input.is_action_pressed("break"):
		world.set_sphere(player.translation + forward() * 20, 5.0, 0)
		world.smooth_sphere(player.translation + forward() * 20, 7.0)


func forward() -> Vector3:
	return (global_transform.origin - player.translation).normalized()
