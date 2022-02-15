extends Label


func _ready():
	var ecs = get_node("/root/ECSController")
	ecs.add_timer(self)
