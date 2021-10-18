#include "EnvelopesAnalysis.hpp"

#include "EnvelopesContainer.hpp"
#include "EnvelopeComparator.hpp"

int InitializeEnvelopesContainer(void** container_to_initialize, double envelope1_size1, double envelope1_size2, double envelope2_size1, double envelope2_size2)
{
    // TODO try smart poriners
    EnvelopesContainer* envelopes_container = NULL;
    
    try
    {
        envelopes_container = new EnvelopesContainer(envelope1_size1, envelope1_size2, envelope2_size1, envelope2_size2);
    }
    catch(const InvalidEnvelopeSizeException& e)
	{
        if (envelopes_container != NULL)
            delete envelopes_container;
        
        return INIT_INVALID_ENVELOPE_SIZE;
	}
        
    *container_to_initialize = (void*)envelopes_container;

    return INIT_SUCCEDED;
}

int CanOneEnvelopeContainAnother(void* container, bool* can_contain)
{
    if (container == NULL)
    {
        return COMPARATION_CONTAINER_IS_NULL;
    }
    
    try
    {
        EnvelopesContainer* envelopes_container = (EnvelopesContainer*)container;

        Envelope* envelope_1 = envelopes_container->get_envelope_1();
        Envelope* envelope_2 = envelopes_container->get_envelope_2();

        *can_contain = EnvelopeComparator::CanOneContainAnother(envelope_1, envelope_2);
    }
    catch(const std::exception& e)
    {
        return COMPARATION_FAILED;
    }

    return COMPARATION_SUCCEDED;
}