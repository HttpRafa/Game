using Godot;

namespace Game.scripts.entities.player;

public partial class AimCrossHair : MeshInstance3D
{

	[Export] private Camera3D _camera3D;
	[Export] private uint _collisionMask;

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
	}

	public override void _PhysicsProcess(double delta)
	{
		PhysicsDirectSpaceState3D spaceState = GetWorld3D().DirectSpaceState;
		var mousePos = GetViewport().GetMousePosition();
		var rayOrigin = _camera3D.ProjectRayOrigin(mousePos);
		var rayEnd = rayOrigin + _camera3D.ProjectRayNormal(mousePos) * 2000;
		var intersection = spaceState.IntersectRay(PhysicsRayQueryParameters3D.Create(rayOrigin, rayEnd, _collisionMask, new Godot.Collections.Array<Rid>()));
		if (intersection.Count > 0)
		{
			Position = intersection["position"].AsVector3();
		}
	}
	
}