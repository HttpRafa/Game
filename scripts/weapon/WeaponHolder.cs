using Game.scripts.entities.player;
using Godot;

namespace Game.scripts.weapon;

public partial class WeaponHolder : Node3D
{

	[Export] private Crosshair _crosshair;
	[Export] private PlayerMovement _playerMovement;

	private Vector3 _rotationTarget;
	private bool _enabled;
	
	public override void _Ready()
	{
		_crosshair.RotationTargetChanged += target =>
		{
			_rotationTarget = target;
		};
		_playerMovement.MouseInputStateChanged += enabled =>
		{
			_enabled = enabled;
		};
	}

	public override void _PhysicsProcess(double delta)
	{
		if(_enabled)
		{
			GlobalTransform = GlobalTransform.LookingAt(_rotationTarget, Vector3.Up);
		}
		else
		{
			Rotation = new Vector3(0f, 0f, 0f);
		}
	}
	
}