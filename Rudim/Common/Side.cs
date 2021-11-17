namespace Rudim.Common
{
    public enum Side
    {
        White,
        Black,
        Both
    }

    public static class SideExtensions
    {
        public static Side Other(this Side side)
        {
            return side switch
            {
                Side.White => Side.Black,
                Side.Black => Side.White,
                _ => Side.Both
            };
        }
    }
}