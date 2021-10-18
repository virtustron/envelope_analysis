#pragma once

const int INIT_SUCCEDED                 = 0;
const int INIT_INVALID_ENVELOPE_SIZE    = 1;

const int COMPARATION_SUCCEDED          = 0;
const int COMPARATION_FAILED            = 1;
const int COMPARATION_CONTAINER_IS_NULL = 2;


int InitializeEnvelopesContainer(void** container_to_initialize, double envelope1_size1, double envelope1_size2, double envelope2_size1, double envelope2_size2);

int CanOneEnvelopeContainAnother(void* container, bool*can_contain);
