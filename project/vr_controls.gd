extends ARVROrigin


export var radius = 8.0

onready var object = $"/root/Main/VoxelObject"

var right_pressed := false
var left_pressed := false

func _ready():
	pass


func _process(_delta):
	if right_pressed:
		var pos = $VRRight.translation
		object.set_sphere(pos, radius, 255)
		object.smooth_sphere(pos, radius+2)

	if left_pressed:
		var pos = $VRLeft.translation
		object.set_sphere(pos, radius, 0)
		object.smooth_sphere(pos, radius+2)


func _on_VRLeft_button_pressed(button):
	if button == JOY_VR_TRIGGER:
		left_pressed = true


func _on_VRLeft_button_release(button):
	if button == JOY_VR_TRIGGER:
		left_pressed = false


func _on_VRRight_button_pressed(button):
	if button == JOY_VR_TRIGGER:
		right_pressed = true


func _on_VRRight_button_release(button):
	if button == JOY_VR_TRIGGER:
		right_pressed = false


