extends ARVROrigin


export var radius_max = 8.0

onready var object = $"/root/Main/VoxelObject"

var right_pressed := false
var left_pressed := false

const TRIGGER = JOY_VR_TRIGGER

func _ready():
	pass



func _physics_process(_delta):
	var right_amount = $VRRight.get_joystick_axis(JOY_VR_ANALOG_TRIGGER)
	if right_amount > 0.25:
		var pos = $VRRight.translation
		var radius = radius_max * right_amount
		object.set_sphere(pos, radius, 255)
		object.smooth_sphere(pos, radius + 2)

	var left_amount = $VRLeft.get_joystick_axis(JOY_VR_ANALOG_TRIGGER)
	if left_amount > 0.99:
		var radius = radius_max
		var pos = $VRLeft.translation
		object.set_sphere(pos, radius, 0)
		object.smooth_sphere(pos, radius+2)

	if Input.is_action_just_pressed("f4"):
		var mesh_r = $VRRight.get_mesh()
		if mesh_r:
			$VRRight/MeshInstance.mesh = mesh_r

func _on_VRLeft_button_pressed(button):
	if button == TRIGGER:
		left_pressed = true


func _on_VRLeft_button_release(button):
	if button == TRIGGER:
		left_pressed = false


func _on_VRRight_button_pressed(button):
	if button == TRIGGER:
		right_pressed = true


func _on_VRRight_button_release(button):
	if button == TRIGGER:
		right_pressed = false


