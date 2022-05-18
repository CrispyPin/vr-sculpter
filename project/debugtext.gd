extends Label

onready var world = $"/root/Main/VoxelObject"
onready var player = $"/root/Main/Camera"
onready var interaction = $"/root/Main/Camera/Interaction"

func _process(_delta):
	text = " FPS: " + str(Engine.get_frames_per_second())
	text += "\n forward: " + str(((interaction.global_transform.origin - player.translation) * 100).round() / 100)
	text += "\n pos: " + str(player.translation.floor())
	text += "\n                                                  "
