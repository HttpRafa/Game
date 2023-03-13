using Godot;

namespace Game.scripts.entities.camera;

public partial class OverlayCamera : Camera3D
{

	[Export] private Camera3D _mainCamera;
	
	public override void _Process(double delta)
	{
		GlobalTransform = _mainCamera.GlobalTransform;
	}
	
}