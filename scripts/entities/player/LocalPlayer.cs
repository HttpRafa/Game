using Godot;

namespace Game.scripts.entities.player;

public partial class LocalPlayer : Node3D
{

	public static RemotePlayer PlayerBody;
	public static LocalPlayer Instance;

	[Export] public Crosshair Crosshair;
	
	public override void _Ready()
	{
		Instance = this;
	}

}