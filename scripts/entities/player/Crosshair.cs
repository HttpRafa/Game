using Godot;

namespace Game.scripts.entities.player;

public partial class Crosshair : Node3D
{

	[Signal]
	public delegate void RotationTargetChangedEventHandler(Vector3 target);
	
	[Export] private Camera3D _camera3D;
	
	
	public override void _Ready()
	{
		// TODO: Add Input.MouseMode = Input.MouseModeEnum.ConfinedHidden;
	}

	public override void _Process(double delta)
	{
		PhysicsDirectSpaceState3D spaceState = GetWorld3D().DirectSpaceState;
		var mousePos = GetViewport().GetMousePosition();
		var rayOrigin = _camera3D.ProjectRayOrigin(mousePos);
		var rayEnd = rayOrigin + _camera3D.ProjectRayNormal(mousePos) * 2000;
		var intersection = spaceState.IntersectRay(PhysicsRayQueryParameters3D.Create(rayOrigin, rayEnd));
		if (intersection.Count > 0)
		{
			Vector3 target = intersection["position"].AsVector3();
			Position = target;
			EmitSignal(SignalName.RotationTargetChanged, target);
		}
	}

}