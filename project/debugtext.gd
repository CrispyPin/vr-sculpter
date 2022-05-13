extends Label

onready var world = $"/root/Main/VoxelObject"
onready var player = $"/root/Main/Camera"
onready var interaction = $"/root/Main/Camera/Interaction"

func _process(_delta):
	text = " FPS: " + str(Engine.get_frames_per_second())
#	text += "\n Load distance: " + str(world.load_distance)
	text += "\n forward: " + str(((interaction.global_transform.origin - player.translation) * 100).round() / 100)
	text += "\n pos: " + str(player.translation.floor())
#	text += "\n voxel type: " + str(interaction.vtype)
#	text += "\n total chunks: " + str(world.chunk_count())
#	text += "\n loaded chunks: " + str(world.loaded_chunk_count())
#	text += "\n empty chunks: " + str(world.empty_chunk_count())
#	text += "\n generating chunks: " + str(world.waiting_chunk_count())
	text += "\n                                                  "
