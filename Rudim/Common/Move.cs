namespace Rudim.Common
{
    public class Move
    {
        public Square Source { get; set; }
        public Square Target { get; set; }
        public MoveType Type { get; set; }

        public Move(Square source, Square target, MoveType type)
        {
            Source = source;
            Target = target;
            Type = type;
        }
    }
}
