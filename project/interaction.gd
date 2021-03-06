extends Spatial

export var enable_highlight = false
export var reach = 15

onready var world = $"/root/Main/VoxelObject"
onready var name_field = $"/root/Main/DebugUI/VBoxContainer/HBoxContainer/LineEdit"
onready var player = $".."


func _ready():
	world.set_sphere(Vector3(0,0,0), 32.0, 255)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)
	world.smooth_sphere(Vector3(0,0,0), 34.0)


func _process(_delta):
	if Input.is_action_just_pressed("f1"):
		world.save()

	if Input.is_action_just_pressed("f2"):
		world.load()

	if Input.is_action_just_pressed("f3"):
		world.export()

	if player.paused:
		return
	else:
		name_field.release_focus()

	if Input.is_action_pressed("place"):
		world.set_sphere(player.translation + forward() * reach, 5.0, 205)
		# world.smooth_sphere(player.translation + forward() * reach, 6.0)

	if Input.is_action_pressed("break"):
#		world.set_sphere(player.translation + forward() * reach, 5.0, 0)
		world.smooth_sphere(player.translation + forward() * reach, 18.0)


func forward() -> Vector3:
	return (global_transform.origin - player.translation).normalized()


func _on_load_pressed():
	world.set_name(name_field.text)
	world.load()


func _on_save_pressed():
	world.set_name(name_field.text)
	world.save()


func _on_export_pressed():
	world.export()
