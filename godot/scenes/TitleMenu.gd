extends Node


func _ready():
	var ecs = get_node("/root/ECSController")
	ecs.add_title_menu(self)
