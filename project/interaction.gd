extends Spatial

export var enable_highlight = false

onready var world = $"/root/Main/VoxelObject"
onready var player = $".."


func _ready():
	world.set_sphere(Vector3(0,0,0), 32.0, 255)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)


func _process(delta):
	if Input.is_action_pressed("place"):
		world.set_sphere(player.translation + forward() * 20, 5.0, 255)
		world.smooth_sphere(player.translation + forward() * 20, 7.0)

	if Input.is_action_pressed("break"):
		world.set_sphere(player.translation + forward() * 20, 5.0, 0)
		world.smooth_sphere(player.translation + forward() * 20, 7.0)


func forward() -> Vector3:
	return (global_transform.origin - player.translation).normalized()
