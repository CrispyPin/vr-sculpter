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


#	var result = raycast()
	var result = {"hit": false}

	if Input.is_action_just_pressed("place"):
		if result.hit:
			if world.get_voxel(result.pos + result.normal*0.5) == 0:
				world.set_voxel(result.pos + result.normal*0.5, vtype)

	if Input.is_action_just_pressed("break"):
		if result.hit:
			world.set_voxel(result.pos - result.normal*0.4, 0)

	if Input.is_action_just_released("next_item"):
		vtype = ((vtype + 255) % 255) + 1
	if Input.is_action_just_released("prev_item"):
		vtype = ((vtype-2 + 255) % 255)+1


func raycast():
	var result = world.cast_ray(player.translation, forward(), 12.0)
	indicator.translation = (result.pos - result.normal*0.5).floor()
	indicator.visible = result.hit && enable_highlight
	return result


func forward() -> Vector3:
	return (global_transform.origin - player.translation).normalized()
