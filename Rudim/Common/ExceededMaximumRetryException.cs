using System;
using System.Runtime.Serialization;

namespace Rudim.Common
{
    [Serializable]
    internal class ExceededMaximumRetryException : Exception
    {
        public ExceededMaximumRetryException()
        {
        }

        public ExceededMaximumRetryException(string message) : base(message)
        {
        }

        public ExceededMaximumRetryException(string message, Exception innerException) : base(message, innerException)
        {
        }

        protected ExceededMaximumRetryException(SerializationInfo info, StreamingContext context) : base(info, context)
        {
        }
    }
}