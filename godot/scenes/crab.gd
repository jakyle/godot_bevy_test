extends KinematicBody2D

export var speed = 10.0

func _ready():
	var ecs = get_node("/root/ECSController")
	ecs.add_node_to_ecs(self, speed, "crab")
